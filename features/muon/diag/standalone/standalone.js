const feature_Standalone = {
  standalone: () => matchMedia('(display-mode: standalone)').matches
    || navigator.standalone === true,
  phone: () => /iPhone|iPad|Android/.test(navigator.userAgent)
    || (navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1),
};
