import { CheckCircle2Icon, InfoIcon } from "lucide-react"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"

export function AlertRtl() {
  return (
    <div dir="rtl" className="grid w-full max-w-md gap-4">
      <Alert>
        <CheckCircle2Icon />
        <AlertTitle>تم الدفع بنجاح</AlertTitle>
        <AlertDescription>تمت معالجة دفعتك البالغة 29.99 دولارًا. تم إرسال إيصال إلى عنوان بريدك الإلكتروني.</AlertDescription>
      </Alert>
      <Alert>
        <InfoIcon />
        <AlertTitle>ميزة جديدة متاحة</AlertTitle>
        <AlertDescription>لقد أضفنا دعم الوضع الداكن. يمكنك تفعيله في إعدادات حسابك.</AlertDescription>
      </Alert>
    </div>
  )
}
