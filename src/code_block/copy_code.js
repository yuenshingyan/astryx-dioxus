const source = await dioxus.recv();
try {
  await navigator.clipboard.writeText(source);
  return true;
} catch {
  return false;
}
