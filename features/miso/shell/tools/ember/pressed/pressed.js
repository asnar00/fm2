// iOS gives :active to a touch only when something listens for touches: one
// passive, empty listener is the whole of what this file does.
document.addEventListener('touchstart', function () {}, { passive: true });
