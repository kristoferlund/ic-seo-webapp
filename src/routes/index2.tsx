import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/index2")({
  component: Index,
});

function Index() {
  return <div className="text-2xl text-white">Current page: /index2</div>;
}
