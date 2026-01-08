import { createFileRoute } from "@tanstack/react-router";
import { useEffect } from "react";

export const Route = createFileRoute("/subpath/$id")({
  component: Index,
});

function Index() {
  const { id } = Route.useParams();

  useEffect(() => {
    document.title = `Subpage ${id}`;
  }, [id]);

  return <div className="text-2xl text-white">Current page: /subpath/{id}</div>;
}
