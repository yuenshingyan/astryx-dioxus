const [id, message] = await dioxus.recv();
const input = document.getElementById(id);
if (input && !input.disabled) {
  const announcer = installAnnounce().createAnnouncer();
  // The shared writer drops this one-shot request if the field disappears before delivery.
  announcer.announce(input, message, () => input.isConnected && !input.disabled);
}
