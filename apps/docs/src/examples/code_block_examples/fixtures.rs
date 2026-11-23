// Displayed snippets from the pinned shadcn/ui examples.

pub(super) const BASIC: &str = r###"$ pnpm build
> next build

 ✓ Compiled successfully in 4.2s
 ✓ Generating static pages (12/12)

Route (app)                Size  First Load JS
┌ ○ /                    1.9 kB         112 kB
├ ○ /_not-found           993 B         103 kB
└ ƒ /api/chat             141 B         102 kB"###;

pub(super) const HEADER: &str = r###"$ pnpm add ai @ai-sdk/react zod
Packages: +14
Progress: resolved 412, reused 398, downloaded 14, added 14, done
Done in 2.8s using pnpm v10.18.0"###;

pub(super) const ACTIONS: &str = r###"$ pnpm lint
apps/web/components/message-list.tsx
  18:22  error  'messages' is possibly 'undefined'. Add a guard before calling map  @typescript-eslint/no-unsafe-call
  42:7   warning  React Hook React.useEffect has a missing dependency: 'scrollToBottom'  react-hooks/exhaustive-deps

✖ 2 problems (1 error, 1 warning)"###;

pub(super) const FLOATING: &str = r###"$ pnpm add ai @ai-sdk/react zod
Progress: resolved 412, reused 398, downloaded 14, added 14, done
dependencies:
+ @ai-sdk/react 3.0.12
+ ai 6.0.12
+ zod 4.1.12
Done in 2.8s using pnpm v10.18.0"###;

pub(super) const ROUTE: &str = r###"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    system: "You are a helpful assistant. Answer in one short paragraph.",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"###;

pub(super) const LINE_NUMBERS: &str = r###"{
  "name": "chat",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build"
  }
}"###;

pub(super) const START: &str = r###"  if (!session) {
    redirect("/login")
  }

  const projects = await db.project.findMany({
    where: { ownerId: session.user.id },
    orderBy: { updatedAt: "desc" },
  })

  return <ProjectList projects={projects} />
}"###;

pub(super) const WRAP: &str = r###"Error: The model "anthropic/claude-sonnet-4.5" returned an error: Request too large. The prompt is 212,431 tokens and the context window is 200,000 tokens. Remove some messages or summarize the conversation and try again."###;

pub(super) const HIGHLIGHTED: &str = r###"export default {
  experimental: {
    reactCompiler: true,
    viewTransition: true,
  },
  images: {
    remotePatterns: [{ hostname: "avatars.githubusercontent.com" }],
  },
}"###;

pub(super) const DIFF: &str = r###" export function Greeting({ name }: { name: string }) {
-  return <p>Hello, {name}</p>
+  return <p className="text-sm text-muted-foreground">
+    Hello, {name}!
+  </p>
 }"###;

pub(super) const SHOW_MORE: &str = r###""use client"

import * as React from "react"
import { useChat } from "@ai-sdk/react"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

export function Chat() {
  const [input, setInput] = React.useState("")
  const { messages, sendMessage, status } = useChat()

  return (
    <div className="flex flex-col gap-4">
      {messages.map((message) => (
        <div key={message.id}>
          {message.parts.map((part, index) =>
            part.type === "text" ? <p key={index}>{part.text}</p> : null
          )}
        </div>
      ))}
      <form
        onSubmit={(event) => {
          event.preventDefault()
          sendMessage({ text: input })
          setInput("")
        }}
      >
        <Input value={input} onChange={(event) => setInput(event.target.value)} />
        <Button disabled={status !== "ready"}>Send</Button>
      </form>
    </div>
  )
}"###;

pub(super) const SIZE: &str = r###"$ pnpm add ai @ai-sdk/react zod
Progress: resolved 412, reused 398, downloaded 14, added 14, done
Done in 2.8s using pnpm v10.18.0"###;

pub(super) const ERROR: &str = r###"TypeError: Cannot read properties of undefined (reading 'map')
    at MessageList (components/message-list.tsx:18:22)
    at renderWithHooks (react-dom.development.js:16305:18)"###;

pub(super) const TRACE: &str = r###"$ node --version
v24.18.0"###;

pub(super) const TRACE_ERROR: &str = r###"$ pnpm dev
Error: listen EADDRINUSE: address already in use :::3000
    at Server.setupListenHandle [as _listen2] (node:net:1908:16)"###;

pub(super) const CHAT: &str = r###"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"###;

pub(super) const TYPESET: &str = r###"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()
  const result = streamText({ model: "anthropic/claude-sonnet-4.5", messages })

  return result.toUIMessageStreamResponse()
}"###;

pub(super) const RTL: &str = r###"pnpm add ai @ai-sdk/react zod
pnpm dev"###;

pub(super) const JSON: &str = r###"{
  "location": "San Francisco",
  "temperature": 18,
  "unit": "celsius",
  "conditions": ["fog", "wind"]
}"###;

pub(super) const ANSI: &str = "$ pnpm test\n\x1b[32m✓\x1b[39m src/format.test.ts \x1b[2m(4 tests)\x1b[22m\n\x1b[32m✓\x1b[39m src/parse.test.ts \x1b[2m(7 tests)\x1b[22m\n\x1b[31m✗\x1b[39m src/stream.test.ts \x1b[2m(3 tests | 1 failed)\x1b[22m\n\n\x1b[31mAssertionError: expected 'hello' to be 'hello world'\x1b[39m\n\nTests  \x1b[31m1 failed\x1b[39m | \x1b[32m13 passed\x1b[39m (14)";

pub(super) const RESPONSE: &str = r###"Wrap the value in a `useDebounce` hook. It returns the latest value once it stops changing for `delay` milliseconds.

```tsx
export function useDebounce<T>(value: T, delay = 300) {
  const [debounced, setDebounced] = React.useState(value)

  React.useEffect(() => {
    const timeout = setTimeout(() => setDebounced(value), delay)
    return () => clearTimeout(timeout)
  }, [value, delay])

  return debounced
}
```

Pass the debounced value to your fetch instead of the raw input."###;

pub(super) const TAB_0: &str = r###"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()
  const result = streamText({ model: "anthropic/claude-sonnet-4.5", messages })

  return result.toUIMessageStreamResponse()
}"###;

pub(super) const TAB_1: &str = r###""use client"

import { useChat } from "@ai-sdk/react"

export function Chat() {
  const { messages } = useChat()

  return <MessageList messages={messages} />
}"###;

pub(super) const TAB_2: &str = r###"AI_GATEWAY_API_KEY=your-api-key"###;

pub(super) const LANG_0: &str = r###"@import "tailwindcss";

:root {
  --radius: 0.625rem;
}"###;

pub(super) const LANG_1: &str = r###"def greet(name: str) -> str:
    return f"Hello, {name}!""###;

pub(super) const LANG_2: &str = r###"A language that is not in the list renders as plain text."###;

pub(super) const STREAM_0: &str = r###"export function useDebounce<T>(value: T, delay = 300) {
  const [debounced, setDebounced] = React.useState(value)

  React.useEffect(() => {
    const timeout = setTimeout(() => setDebounced(value), delay)
    return () => clearTimeout(timeout)
  }, [value, delay])

  return debounced
}"###;

pub(super) const STREAM_1: &str = r###"pnpm dlx shadcn@latest add code-block"###;

pub(super) const STREAM_2: &str = r###" export function Search() {
   const [query, setQuery] = React.useState("")
-  const results = useSearch(query)
+  const debounced = useDebounce(query, 300)
+  const results = useSearch(debounced)

   return <SearchResults results={results} />
 }"###;

pub(super) const STREAM_3: &str = r###"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"###;

pub(super) const STREAM_4: &str = r###"AI_GATEWAY_API_KEY=your-api-key"###;
