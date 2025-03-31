import { useMutation } from "@tanstack/react-query";
import { server } from "../server/declarations/index";

export default function useGreet(onSuccess: (data: string) => void) {
  return useMutation({
    mutationFn: (name: string) => {
      return server.greet(name);
    },
    onSuccess,
  });
}
