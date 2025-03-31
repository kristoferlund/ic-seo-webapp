import GreetForm from "@/components/greet-form";
import icLogo from "../assets/ic.svg";
import { createRootRoute, Link, Outlet } from "@tanstack/react-router";

export const Route = createRootRoute({
  component: () => (
    <main className="dark">
      <div className="flex flex-col gap-14 items-center">
        <div className="flex gap-10">
          <a
            href="https://internetcomputer.org"
            target="_blank"
            rel="noreferrer"
          >
            <img src={icLogo} alt="ICP logo" className="h-20" />
          </a>
        </div>
        <div className="flex gap-5 text-2xl text-white">
          Menu:
          <Link to="/" className="underline">
            index
          </Link>
          <Link to="/index2" className="underline">
            index2
          </Link>
          <Link to="/subpath/$id" params={{ id: "one" }} className="underline">
            subpath/one
          </Link>
          <Link to="/subpath/$id" params={{ id: "two" }} className="underline">
            subpath/two
          </Link>
        </div>
        <Outlet />
        <GreetForm />
      </div>
    </main>
  ),
});
