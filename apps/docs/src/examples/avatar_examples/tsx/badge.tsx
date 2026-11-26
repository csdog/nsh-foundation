import { Avatar } from "nsh-foundation/native";

export function Example() {
  return (
    <Avatar name="@shadcn" src="asset:/@docs/avatar-shadcn.jpg" fallback="CN" badgeStatus="Online" badgeColor="#16a34a" />
  );
}
