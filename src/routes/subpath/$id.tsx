import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/subpath/$id")({
  component: Index,
});

function Index() {
  const { id } = Route.useParams();

  return <div className="text-2xl text-white">Current page: /subpath/{id}</div>;
}
