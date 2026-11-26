import { mount, signal, View } from "nsh";
import { WebGallery } from "nsh-foundation/native";

import { examples } from "./.nsh/examples.generated";

function App() {
  const selected = signal("");
  const renderExample = () => {
    const Example = examples.get(selected());
    return Example ? <Example /> : null;
  };

  return (
    <WebGallery
      onRunTsx={({ id }) => selected.set(id)}
      example={<View>{renderExample()}</View>}
    />
  );
}

mount(App);
