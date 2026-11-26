import { Text, View } from "nsh";
import {
  DrawerClose,
  DrawerContent,
  DrawerRoot,
  DrawerTrigger,
} from "nsh-foundation/native";

export function DrawerExample() {
  return (
    <DrawerRoot>
      <DrawerTrigger>
        <Text>Open Drawer</Text>
      </DrawerTrigger>
      <DrawerContent accessibilityLabel="Move Goal">
        <View style={{ padding: 16, gap: 8 }}>
          <Text style={{ fontSize: 20 }}>Move Goal</Text>
          <Text>Set your daily activity goal.</Text>
          <DrawerClose>
            <Text class="text-primary-foreground">Done</Text>
          </DrawerClose>
        </View>
      </DrawerContent>
    </DrawerRoot>
  );
}
