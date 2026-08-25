# plus-tinted
*the person-with-a-plus is black on a palette colour*

> (asks#1787668059398)
> add user icon should be in house style black on colour, no white
> *(filed from the field on 2026-08-25 by ash)*

## user

The invite sub-tool button looks like the other tool buttons: a black person-with-a-plus on one of the toolbar's colours, whether it sits in 👤's row or on its own page.

## spec

`/under-account` draws the invite sub-tool as a plain control — white on dark grey. Ash asked for house style: black on a palette colour. One reading, so it builds. This node extends `tool_controls` and gives the `tool_invite` button `/ember`'s `tinted` class and `--tool-colour`, with the colour from `tool_colour("invite")` — `/ember`'s stable pick for the name. The emoji goes black through `/ember`'s icon filter; the drawn `+` inherits black from one rule here. With `/ember` unticked the colour is empty and the button is left as drawn. Untick this node and the plain control returns.

## glossary

(no new terms)

## code description

`plus-tinted.rs` — `tool_controls` calls `existing` and rewrites the invite button's opening tag with `tinted` and the colour variable (the `/tinted` idiom).

`plus-tinted.css` — black text on the tinted invite button, so the `+` badge is black too.
