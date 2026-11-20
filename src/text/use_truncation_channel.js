// Dioxus renderer lifetime; measurement remains with Text/useTruncation.
const controller = createTruncation((isTruncated, fullText) => dioxus.send([isTruncated, fullText]));
try {
  while (true) {
    const config = await dioxus.recv();
    if (config == null || config === 'dispose') break;
    const [id, maxLines, isExpanded] = config;
    const element = document.querySelector(`[data-astryx-truncation-id="${CSS.escape(id)}"]`);
    controller.attach(element, maxLines, isExpanded);
  }
} finally {
  controller.dispose();
}
