async function main() {
  const { greet } = await import("@kojiro.ueda/lib");
  greet("hello world");
}

main();
