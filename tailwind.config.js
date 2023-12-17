module.exports = {
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  darkMode: "media", // 'media' or 'class'
  theme: {
    extend: {
      fontFamily: {
        poppins: "Poppins",
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
