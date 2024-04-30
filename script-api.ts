type Baz = {
  baz: string;
  boo: number;
};

export function baz(baz: string): Baz {
  return {
    baz: baz ?? "",
    boo: 42,
  };
}

type ToReturn = {
  title: string;
  message: string;
  baz: Baz;
};

export function toReturn({ title, message, baz }: Partial<ToReturn>): ToReturn {
  return {
    title: title ?? "Hello",
    message: message ?? "World",
    baz: baz ?? { baz: "baz", boo: 33 },
  };
}
