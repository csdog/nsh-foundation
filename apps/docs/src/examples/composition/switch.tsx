import { Text, View, signal } from "nsh";
import { Switch } from "nsh-foundation/native";

export function SwitchExample() {
  const notifications = signal(false);
  return (
    <View style={{ gap: 8 }}>
      <Text>Notifications</Text>
      <Switch
        accessibilityLabel="Notifications"
        checked={notifications()}
        onChange={({ checked }) => notifications.set(checked)}
      />
      <Text>{notifications() ? "On" : "Off"}</Text>
    </View>
  );
}
