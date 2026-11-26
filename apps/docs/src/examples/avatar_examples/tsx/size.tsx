import { View } from "nsh";
import { Avatar } from "nsh-foundation/native";

export function Example() {
  return (
    <View style={{ flexDirection: "row", alignItems: "center", gap: 16 }}>
      <Avatar name="@shadcn" src="asset:/@docs/avatar-shadcn.jpg" fallback="CN" size="small" />
      <Avatar name="@shadcn" src="asset:/@docs/avatar-shadcn.jpg" fallback="CN" />
      <Avatar name="@shadcn" src="asset:/@docs/avatar-shadcn.jpg" fallback="CN" size="large" />
    </View>
  );
}
