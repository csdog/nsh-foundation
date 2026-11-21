"use client"

import * as React from "react"
import {
  ArchiveIcon,
  ArrowLeftIcon,
  CalendarPlusIcon,
  ClockIcon,
  ListFilterIcon,
  MailCheckIcon,
  MoreHorizontalIcon,
  TagIcon,
  Trash2Icon,
} from "lucide-react"

import { Button } from "@/components/ui/button"
import { ButtonGroup } from "@/components/ui/button-group"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"

export function ButtonGroupRtl() {
  const [label, setLabel] = React.useState("personal")

  return (
    <div dir="rtl">
      <ButtonGroup>
        <ButtonGroup className="hidden sm:flex">
          <Button variant="outline" size="icon" aria-label="Go Back">
            <ArrowLeftIcon className="rtl:rotate-180" />
          </Button>
        </ButtonGroup>
        <ButtonGroup>
          <Button variant="outline">أرشفة</Button>
          <Button variant="outline">تقرير</Button>
        </ButtonGroup>
        <ButtonGroup>
          <Button variant="outline">تأجيل</Button>
          <DropdownMenu>
            <DropdownMenuTrigger
              render={
                <Button
                  variant="outline"
                  size="icon"
                  aria-label="More Options"
                />
              }
            >
              <MoreHorizontalIcon />
            </DropdownMenuTrigger>
            <DropdownMenuContent
              align="start"
              dir="rtl"
              className="w-40"
            >
              <DropdownMenuGroup>
                <DropdownMenuItem>
                  <MailCheckIcon />
                  وضع علامة كمقروء
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <ArchiveIcon />
                  أرشفة
                </DropdownMenuItem>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuGroup>
                <DropdownMenuItem>
                  <ClockIcon />
                  تأجيل
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <CalendarPlusIcon />
                  إضافة إلى التقويم
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <ListFilterIcon />
                  إضافة إلى القائمة
                </DropdownMenuItem>
                <DropdownMenuSub>
                  <DropdownMenuSubTrigger>
                    <TagIcon />
                    تصنيف كـ…
                  </DropdownMenuSubTrigger>
                  <DropdownMenuSubContent
                    dir="rtl"
                  >
                    <DropdownMenuRadioGroup
                      value={label}
                      onValueChange={setLabel}
                    >
                      <DropdownMenuRadioItem value="personal">
                        شخصي
                      </DropdownMenuRadioItem>
                      <DropdownMenuRadioItem value="work">
                        عمل
                      </DropdownMenuRadioItem>
                      <DropdownMenuRadioItem value="other">
                        آخر
                      </DropdownMenuRadioItem>
                    </DropdownMenuRadioGroup>
                  </DropdownMenuSubContent>
                </DropdownMenuSub>
              </DropdownMenuGroup>
              <DropdownMenuSeparator />
              <DropdownMenuGroup>
                <DropdownMenuItem variant="destructive">
                  <Trash2Icon />
                  سلة المهملات
                </DropdownMenuItem>
              </DropdownMenuGroup>
            </DropdownMenuContent>
          </DropdownMenu>
        </ButtonGroup>
      </ButtonGroup>
    </div>
  )
}
