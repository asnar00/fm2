// the wire got roomier, so the card budget follows it: a 384px picture at up
// to 24KB, and 56KB for the whole cards list — under the 64KB message cap
// with room for the op's own envelope. Set at load, read at use, so the
// numbers leave with this node.
if (typeof feature_Cards !== 'undefined') {
  feature_Cards.CAP = 24576;
  feature_Cards.LIST_CAP = 56000;
  feature_Cards.EDGE = 384;
}
