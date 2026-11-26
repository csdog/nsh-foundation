import { Text, View, signal } from "nsh";
import { Button } from "nsh-foundation/native";

export function ButtonExample() {
  const saved = signal(false);
  return (
    <View style={{ gap: 8 }}>
      <Button onPress={() => saved.set(true)} style={{ paddingHorizontal: 16 }}>
        <Text>Save</Text>
      </Button>
      <Text>{saved() ? "Saved." : ""}</Text>
    </View>
  );
}
