import { Avatar, AvatarGroup } from "nsh-foundation/native";

export function Example() {
  return (
    <AvatarGroup>
      <Avatar name="@shadcn" src="asset:/@docs/avatar-shadcn.jpg" fallback="CN" grayscale />
      <Avatar name="@maxleiter" src="asset:/@docs/avatar-maxleiter.png" fallback="LR" grayscale />
      <Avatar name="@evilrabbit" src="asset:/@docs/avatar-evilrabbit.png" fallback="ER" grayscale />
    </AvatarGroup>
  );
}
