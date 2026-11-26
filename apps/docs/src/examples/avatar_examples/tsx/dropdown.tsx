import { Text, View, signal } from "nsh";
import { Menu, MenuItem, MenuSeparator } from "nsh/base";
import { Avatar } from "nsh-foundation/native";

export function Example() {
  const selected = signal("");
  return (
    <View class="items-center gap-2">
      <Menu
        offset={6}
        margin={8}
        popupClass="w-32 p-1 bg-popover text-popover-foreground rounded-lg border shadow-md"
        trigger={
          <View role="button" accessibilityLabel="Account menu" class="rounded-full">
            <Avatar name="shadcn" src="asset:/@docs/avatar-shadcn.jpg" fallback="CN" />
          </View>
        }
      >
        <MenuItem onSelect={() => selected.set("Profile")}><Text>Profile</Text></MenuItem>
        <MenuItem onSelect={() => selected.set("Billing")}><Text>Billing</Text></MenuItem>
        <MenuItem onSelect={() => selected.set("Settings")}><Text>Settings</Text></MenuItem>
        <MenuSeparator />
        <MenuItem onSelect={() => selected.set("Log out")}><Text class="text-destructive">Log out</Text></MenuItem>
      </Menu>
      {selected() && <Text>Selected in this preview: {selected()}</Text>}
    </View>
  );
}
