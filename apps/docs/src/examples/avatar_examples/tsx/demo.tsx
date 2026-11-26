import { Text, View } from "nsh";
import { Avatar, AvatarGroup, AvatarGroupCount } from "nsh-foundation/native";

export function Example() {
  return (
    <View style={{ flexDirection: "row", alignItems: "center", gap: 16 }}>
      <Avatar name="@shadcn" src="asset:/@docs/avatar-shadcn.jpg" fallback="CN" />
      <Avatar name="@evilrabbit" src="asset:/@docs/avatar-evilrabbit.png" fallback="ER" badgeStatus="Online" badgeColor="#16a34a" />
      <AvatarGroup>
        <Avatar name="@shadcn" src="asset:/@docs/avatar-shadcn.jpg" fallback="CN" grayscale />
        <Avatar name="@maxleiter" src="asset:/@docs/avatar-maxleiter.png" fallback="LR" grayscale />
        <Avatar name="@evilrabbit" src="asset:/@docs/avatar-evilrabbit.png" fallback="ER" grayscale />
        <AvatarGroupCount accessibilityLabel="3 more people"><Text style={{ fontSize: 14 }}>+3</Text></AvatarGroupCount>
      </AvatarGroup>
    </View>
  );
}
