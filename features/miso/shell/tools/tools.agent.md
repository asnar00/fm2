# tools — the interface is a tree of tools

*You are adding or changing something a person taps. This instruction
toggles with the `/tools` node.*

miso's interface is a tree. The toolbar is the root. A tool opens a level.
A tool's actions are its **sub-tools**, drawn as buttons in the control row
beside the tool's own icon — as taps carries reset and −1, as 👤 carries the
plus. A sub-tool may open a level of its own, with its own row. ‹ goes to
the parent level, always exactly one, however deep. The row shows the
current level's icon on the left and never the parent chain.

So: never put buttons on a page to choose between actions — a page with
buttons is a toolbar in disguise (ash, 2026-09-02, on the invite page's two
buttons: "the page with the two buttons is doing the job of the toolbar").
Make them sub-tools. A sheet is for entering things (a name and a number),
not for choosing what to do. Undo stands apart at the far right when there
is a step; the long-press card says what each button is for.
