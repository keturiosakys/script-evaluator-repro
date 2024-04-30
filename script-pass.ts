import {
	baz,
  toReturn
} from "./script-api.ts";

export default function main() {
  return toReturn({
    title: "Hello",
		message: "world",
		baz: baz("baz")
  });
}

