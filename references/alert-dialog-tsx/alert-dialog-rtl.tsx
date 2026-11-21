import { BluetoothIcon } from "lucide-react"

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogMedia,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { Button } from "@/components/ui/button"

export function AlertDialogRtl() {
  return (
    <div dir="rtl" className="flex flex-wrap gap-4">
      <AlertDialog>
        <AlertDialogTrigger render={<Button variant="outline" />}>إظهار الحوار</AlertDialogTrigger>
        <AlertDialogContent dir="rtl">
          <AlertDialogHeader>
            <AlertDialogTitle>هل أنت متأكد تمامًا؟</AlertDialogTitle>
            <AlertDialogDescription>لا يمكن التراجع عن هذا الإجراء. سيؤدي هذا إلى حذف حسابك نهائيًا من خوادمنا.</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>إلغاء</AlertDialogCancel>
            <AlertDialogAction>متابعة</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
      <AlertDialog>
        <AlertDialogTrigger render={<Button variant="outline" />}>إظهار الحوار (صغير)</AlertDialogTrigger>
        <AlertDialogContent dir="rtl" size="sm">
          <AlertDialogHeader>
            <AlertDialogMedia><BluetoothIcon /></AlertDialogMedia>
            <AlertDialogTitle>السماح للملحق بالاتصال؟</AlertDialogTitle>
            <AlertDialogDescription>هل تريد السماح لملحق USB بالاتصال بهذا الجهاز؟</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>عدم السماح</AlertDialogCancel>
            <AlertDialogAction>السماح</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  )
}
