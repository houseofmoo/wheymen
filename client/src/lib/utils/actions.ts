export function cardClicked(node) {
    const mouseEnter = (node) => {
        console.log("node: ", node);
    };

    document.addEventListener("mouseenter", mouseEnter);

    return {
        destroy() {
            document.removeEventListener("mouseenter", mouseEnter);
        }
    };
}