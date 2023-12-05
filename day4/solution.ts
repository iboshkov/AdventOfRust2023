
const path = "./input.txt";
const file = Bun.file(path);
const input = await file.text();

const processLine = (line: string) => {
    const [id, numbers] = line.split(": ");
    const [winningNumbers, myNumbers] = numbers
      .split(" | ")
      .map((nums) => nums.split(" ").map(Number).filter(Boolean));
  
    const intersection = winningNumbers.filter((num) => myNumbers.includes(num));
    const totalWinning = intersection.length;
  
    return {
      id: id.replace(/\D/g, ''),
      totalWinning,
    };
  };
  
  const partOne = (input: string) => {
    let result = 0;
    const lines = input.split("\n");
  
    lines.forEach((line) => {
      const { totalWinning } = processLine(line);
      result += totalWinning ? 2 ** (totalWinning - 1) : 0;
    });
  
    console.log(`A-SIDE PUZZLE: ${result}`);
  };
  
  
  const partTwo = (input: string) => {
    let result = 0;
    const lines = input.split("\n");
  
    const cards = lines.map(processLine);
  
    const totalCards = cards.reduce<Record<string, number>>((acc, card) => {
      acc[card.id] = 1;
      return acc;
    }, {});
  
    for (const card of cards) {
      const multiplier = totalCards[card.id];
      for (let i = 1; i <= card.totalWinning; i++) {
        totalCards[Number(card.id) + i] += multiplier;
      }
    }
  
    result = Object.values(totalCards).reduce((total, cardTotal) => total + cardTotal, 0);
  
    console.log(`B-SIDE PUZZLE: ${result}`);
  };
  
  partOne(input);
  partTwo(input);