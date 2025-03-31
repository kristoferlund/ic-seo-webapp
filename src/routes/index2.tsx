import { createFileRoute } from "@tanstack/react-router";
import { useEffect } from "react";

export const Route = createFileRoute("/index2")({
  component: Index,
});

function Index() {
  useEffect(() => {
    document.title = "Second index page";
  }, []);

  return <div className="text-2xl text-white">Current page: /index2</div>;
}
