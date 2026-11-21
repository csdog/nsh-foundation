# Upstream references

- [shadcn/ui source](https://github.com/shadcn-ui/ui), MIT. The local checkout is `references/shadcn-ui/`, commit `f50887ae68d0fdcbfbe86b57be08af94a4c88a68`.
- [Base UI Drawer docs](https://ui.shadcn.com/docs/components/base/drawer). In the checkout, see `apps/v4/content/docs/components/base/drawer.mdx`, `apps/v4/registry/bases/base/ui/drawer.tsx` and `apps/v4/examples/base/drawer-*.tsx`.
- [Base UI Accordion docs](https://ui.shadcn.com/docs/components/base/accordion). The [Accordion TSX tabs](accordion-tsx/) use the seven examples from the pinned checkout under the [MIT license](accordion-tsx/LICENSE.md). The imports use `@/components/ui`, and the RTL example sets `dir` directly.
- [Obra shadcn/ui Community Figma kit](https://www.figma.com/community/file/1514746685758799870/obra-shadcn-ui), by Obra Studio, CC BY 4.0 per the [publisher](https://shadcn.obra.studio/products/obra-shadcn-ui-community). The publisher’s public cover, component screenshot, and theme-variable screenshot are saved in `figma/` as visual references. The editable community file could not be exported because this environment's network proxy denied access to Figma. The free Community edition covers Nova, while the Drawer examples use Rhea. Use the upstream code and browser example to verify Rhea details.

The [Drawer TSX examples](drawer-tsx/) are embedded as text in the Rust docs page. The shadcn examples and media-query hook are adapted from the commit above under the [MIT license](drawer-tsx/LICENSE.md). Their imports use the usual `@/components/ui` and `@/lib/utils` aliases. The Custom Sizes and Styling examples are written for this gallery.

To retrieve the source after cloning this project:

```sh
git clone https://github.com/shadcn-ui/ui references/shadcn-ui
git -C references/shadcn-ui checkout f50887ae68d0fdcbfbe86b57be08af94a4c88a68
```
