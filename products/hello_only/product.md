# hello_only
*just hello, no goodbye*

> (transcripts/2026-08-13-fm-spec.md#p20)
> let's do product subsetting next - I guess we'd have one hello that just does hello, another that also does goodbye?

Demonstrates a product-local override: instead of symlinking the `hello` folder (which would import `goodbye`), the product has its own `hello/` folder whose files are symlinks to the shared feature, plus a local `order.md` that unticks `goodbye`.
