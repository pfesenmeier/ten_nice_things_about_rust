const goodTwin = { is: "good" };
const evilTwin = goodTwin;
evilTwin.is = "evil";

// good or evil???
console.log("The good twin:", goodTwin);
