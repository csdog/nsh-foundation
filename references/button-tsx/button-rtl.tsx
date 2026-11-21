import { ArrowRightIcon, PlusIcon } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Spinner } from "@/components/ui/spinner"

export function ButtonRtl() {

  return (
    <div className="flex flex-wrap items-center gap-2 md:flex-row" dir="rtl">
      <Button variant="outline">زر</Button>
      <Button variant="destructive">حذف</Button>
      <Button variant="outline">
        إرسال{" "}
        <ArrowRightIcon className="rtl:rotate-180" data-icon="inline-end" />
      </Button>
      <Button variant="outline" size="icon" aria-label="Add">
        <PlusIcon />
      </Button>
      <Button variant="secondary" disabled>
        <Spinner data-icon="inline-start" /> جاري التحميل
      </Button>
    </div>
  )
}
