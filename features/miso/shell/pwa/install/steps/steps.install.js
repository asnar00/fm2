const feature_Steps = { active: true };
if (/Android/.test(navigator.userAgent)) {
  const ios = document.getElementById('ios');
  const android = document.getElementById('android');
  if (ios) ios.style.display = 'none';
  if (android) android.style.display = 'block';
}
