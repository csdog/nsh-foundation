"use client"

import { Button } from "@/components/ui/button"
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "@/components/ui/drawer"

const updates = [
  "Order confirmed",
  "Preparing your order",
  "Driver assigned",
  "Picked up",
  "Arriving soon",
  "Delivered",
]

export function DrawerCustomSize() {
  return (
    <Drawer>
      <DrawerTrigger render={<Button variant="outline" />}>
        Open Scrollable Drawer
      </DrawerTrigger>
      <DrawerContent className="h-[360px]">
        <DrawerHeader>
          <DrawerTitle>Notification history</DrawerTitle>
          <DrawerDescription>Recent order updates.</DrawerDescription>
        </DrawerHeader>
        <ul className="min-h-0 flex-1 space-y-3 overflow-y-auto p-4">
          {updates.map((update) => (
            <li key={update} className="rounded-lg bg-muted p-3">
              {update}
            </li>
          ))}
        </ul>
        <DrawerFooter>
          <DrawerClose render={<Button />}>Done</DrawerClose>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  )
}
