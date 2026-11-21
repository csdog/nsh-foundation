import { buttonVariants } from "@/components/ui/button"

export default function ButtonRender() {
  return (
    <a
      href="https://ui.shadcn.com/docs/components/base/button#as-link"
      className={buttonVariants({ variant: "secondary", size: "sm" })}
    >
      Login
    </a>
  )
}
