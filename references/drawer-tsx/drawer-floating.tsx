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

export function DrawerFloating() {
  return (
    <Drawer swipeDirection="right">
      <DrawerTrigger render={<Button variant="outline" />}>
        Open Floating Drawer
      </DrawerTrigger>
      <DrawerContent className="w-[360px] rounded-2xl [--drawer-inset:16px]">
        <DrawerHeader>
          <DrawerTitle>Floating Drawer</DrawerTitle>
          <DrawerDescription>A 16 pixel inset on every side.</DrawerDescription>
        </DrawerHeader>
        <DrawerFooter className="mt-auto">
          <DrawerClose render={<Button />}>Close</DrawerClose>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  )
}
