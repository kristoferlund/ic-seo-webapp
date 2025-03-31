import { createFileRoute } from "@tanstack/react-router";
import { useEffect } from "react";

export const Route = createFileRoute("/")({
  component: Index,
});

function Index() {
  useEffect(() => {
    document.title = "Main index page";
  }, []);

  return <div className="text-2xl text-white">Current page: /index</div>;
}
