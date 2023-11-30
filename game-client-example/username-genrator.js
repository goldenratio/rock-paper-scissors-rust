/**
 * Generates a random username.
 *
 * @returns {string} A randomly generated username.
 */
export function generateUsername() {
  const adjectives = ["happy", "silly", "clever", "brave", "funny", "kind", "smart", "gentle", "creative", "honest"];
  const nouns = ["cat", "dog", "bird", "lion", "tiger", "elephant", "monkey", "panda", "dolphin", "butterfly"];

  const randomAdjective = adjectives[Math.floor(Math.random() * adjectives.length)];
  const randomNoun = nouns[Math.floor(Math.random() * nouns.length)];

  return randomAdjective + "_" + randomNoun + "_" + getRandomNumber(1000, 9999);
}

/**
 * Generates a random number between a minimum and maximum value (inclusive).
 *
 * @param {number} min - The minimum value.
 * @param {number} max - The maximum value.
 * @returns {number} A random number between the minimum and maximum value.
 */
function getRandomNumber(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

