const feature_Compute = {
  _device: undefined, // undefined = not yet tried; null = no webgpu here
  pipelines: {},      // wgsl source text -> compiled pipeline

  async init() {
    if (this._device !== undefined) return this._device;
    try {
      if (!navigator.gpu) { this._device = null; return null; }
      const adapter = await navigator.gpu.requestAdapter();
      if (!adapter) { this._device = null; return null; }
      // ask for exactly what the adapter offers — a fixed maximum is the
      // iOS Safari door-slam; the adapter's own numbers always open
      const wanted = ['maxBufferSize', 'maxStorageBufferBindingSize',
        'maxComputeWorkgroupStorageSize', 'maxComputeInvocationsPerWorkgroup'];
      const requiredLimits = {};
      for (const k of wanted) {
        if (adapter.limits[k]) requiredLimits[k] = adapter.limits[k];
      }
      const device = await adapter.requestDevice({ requiredLimits });
      device.lost.then(() => {
        // a lost device is not absence: forget it and re-acquire next run
        this._device = undefined;
        this.pipelines = {};
      });
      this._device = device;
    } catch (e) {
      this._device = null;
    }
    return this._device;
  },

  available() { return !!this._device; },

  // one kernel, one answer: inputs bind as storage buffers in order, the
  // output binds last; entry point 'main', @workgroup_size(64) by
  // convention (kernels guard their own bounds). null = fall to CPU.
  async run(wgsl, inputs, outWords) {
    try {
      const device = await this.init();
      if (!device) return null;
      let pipeline = this.pipelines[wgsl];
      if (!pipeline) {
        pipeline = device.createComputePipeline({
          layout: 'auto',
          compute: { module: device.createShaderModule({ code: wgsl }),
                     entryPoint: 'main' },
        });
        this.pipelines[wgsl] = pipeline;
      }
      const entries = [];
      const buffers = inputs.map((arr, i) => {
        const buf = device.createBuffer({
          size: Math.max(arr.byteLength, 4),
          usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
        });
        device.queue.writeBuffer(buf, 0, arr);
        entries.push({ binding: i, resource: { buffer: buf } });
        return buf;
      });
      const outBytes = Math.max(outWords * 4, 4);
      const out = device.createBuffer({
        size: outBytes,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
      });
      entries.push({ binding: inputs.length, resource: { buffer: out } });
      const staging = device.createBuffer({
        size: outBytes,
        usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
      });
      const bindGroup = device.createBindGroup({
        layout: pipeline.getBindGroupLayout(0), entries });
      const enc = device.createCommandEncoder();
      const pass = enc.beginComputePass();
      pass.setPipeline(pipeline);
      pass.setBindGroup(0, bindGroup);
      pass.dispatchWorkgroups(Math.ceil(outWords / 64));
      pass.end();
      enc.copyBufferToBuffer(out, 0, staging, 0, outBytes);
      device.queue.submit([enc.finish()]);
      await staging.mapAsync(GPUMapMode.READ);
      const result = new Float32Array(staging.getMappedRange().slice(0));
      staging.unmap();
      for (const b of buffers) b.destroy();
      out.destroy(); staging.destroy();
      return result.subarray(0, outWords);
    } catch (e) {
      return null;
    }
  },
};
