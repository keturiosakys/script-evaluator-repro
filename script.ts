import {
	baz,
  toReturn
} from "./script-api.ts";

type MainParams = {
  title: string;
  message: string;
};

export default function main({ title, message }: MainParams) {
  return toReturn({
    title,
		message,
		baz: baz("baz")
  });
}

