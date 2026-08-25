// with /me on, the nøøb panel is not the 👤 tool's sheet any more, so
// dismissing it must not leave the tool: take /account's dismissal seam
// and make it do nothing. Replaced at load; /account's wrap reads it late.
if (typeof feature_Account !== 'undefined') {
  feature_Account.dismissed = function () {};
}
