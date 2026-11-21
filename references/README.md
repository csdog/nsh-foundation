# Upstream references

- [shadcn/ui source](https://github.com/shadcn-ui/ui), MIT. The local checkout is `references/shadcn-ui/`, commit `f50887ae68d0fdcbfbe86b57be08af94a4c88a68`.
- [Base UI Drawer docs](https://ui.shadcn.com/docs/components/base/drawer). In the checkout, see `apps/v4/content/docs/components/base/drawer.mdx`, `apps/v4/registry/bases/base/ui/drawer.tsx` and `apps/v4/examples/base/drawer-*.tsx`.
- [Obra shadcn/ui Community Figma kit](https://www.figma.com/community/file/1514746685758799870/obra-shadcn-ui), by Obra Studio, CC BY 4.0 per the [publisher](https://shadcn.obra.studio/products/obra-shadcn-ui-community). The publisher’s public cover, component screenshot, and theme-variable screenshot are saved in `figma/` as visual references. The editable community file could not be exported because this environment's network proxy denied access to Figma. The free Community edition covers Nova, while the Drawer examples use Rhea. Use the upstream code and browser example to verify Rhea details.

To retrieve the source after cloning this project:

```sh
git clone https://github.com/shadcn-ui/ui references/shadcn-ui
git -C references/shadcn-ui checkout f50887ae68d0fdcbfbe86b57be08af94a4c88a68
```
