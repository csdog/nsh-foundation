import { BadgeCheck, BookmarkIcon } from "lucide-react"
import { Badge } from "@/components/ui/badge"

export function BadgeRtl() {
  return (
    <div className="flex w-full flex-wrap justify-center gap-2" dir="rtl">
      <Badge>شارة</Badge>
      <Badge variant="secondary">ثانوي</Badge>
      <Badge variant="destructive">مدمر</Badge>
      <Badge variant="outline">مخطط</Badge>
      <Badge variant="secondary">
        <BadgeCheck data-icon="inline-start" />
        متحقق
      </Badge>
      <Badge variant="outline">
        إشارة مرجعية
        <BookmarkIcon data-icon="inline-end" />
      </Badge>
    </div>
  )
}
