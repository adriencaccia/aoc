import { readFileSync } from "fs";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

class LinkedListNode {
  data;
  next: LinkedListNode;
  previous: LinkedListNode;
  constructor(
    data: { value: number; index: number },
    next: LinkedListNode | null = null,
    previous: LinkedListNode | null = null
  ) {
    this.data = data;
    // @ts-expect-error
    this.next = next;
    // @ts-expect-error
    this.previous = previous;
  }
}

function isDataEqual(
  a: { value: number; index: number },
  b: { value: number; index: number }
) {
  return a.index === b.index && a.value === b.value;
}

class LinkedList {
  head: LinkedListNode | null;
  constructor() {
    this.head = null;
  }
  insertAtEnd(data: { value: number; index: number }) {
    // A newNode object is created with property data and next=null

    let newNode = new LinkedListNode(data);
    // When head = null i.e. the list is empty, then head itself will point to the newNode.
    if (!this.head) {
      this.head = newNode;
      return this.head;
    }
    // Else, traverse the list to find the tail (the tail node will initially be pointing at null), and update the tail's next pointer.
    let tail = this.head;
    while (tail.next !== null) {
      tail = tail.next;
    }
    tail.next = newNode;

    // modify previous nodes
    newNode.previous = tail;
    this.head.previous = newNode;
  }

  getAt(index: number) {
    let counter = 0;
    let node = this.head;
    while (node) {
      if (counter === index) {
        return node;
      }
      counter++;
      node = node.next;
    }
    return null;
  }

  insertAt(data: { value: number; index: number }, index: number) {
    // if the list is empty i.e. head = null
    if (!this.head) {
      this.head = new LinkedListNode(data);
      return;
    }
    // if new node needs to be inserted at the front of the list i.e. before the head.
    if (index === 0) {
      this.head = new LinkedListNode(data, this.head);
      return;
    }

    // else, use getAt() to find the previous node.
    const previous = this.getAt(index - 1) as LinkedListNode;
    let newNode = new LinkedListNode(data);
    newNode.next = previous.next;
    previous.next = newNode;

    return this.head;
  }

  findIndex(data: { value: number; index: number }) {
    let index = 0;
    let node = this.head;
    if (node === null) {
      throw new Error("List is empty");
    }
    while (!isDataEqual(node.data, data)) {
      if (node.next === null) {
        throw new Error(`${data} was not found in the list`);
      }
      node = node.next;
      index++;
    }
    return index;
  }

  deleteAt(index: number) {
    // when list is empty i.e. head = null
    if (!this.head) {
      return;
    }
    // node needs to be deleted from the front of the list i.e. before the head.
    if (index === 0) {
      this.head = this.head.next;
      return;
    }
    // else, use getAt() to find the previous node.
    const previous = this.getAt(index - 1);

    if (!previous || !previous.next) {
      return;
    }

    previous.next = previous.next.next;
    return this.head;
  }

  printList(length: number) {
    let list: number[] = [];
    let node = this.head;
    for (let i = 0; i < length; i++) {
      if (node?.next == null) {
        break;
      }
      list.push(node.data.value);
      node = node.next;
    }
    console.log(list.join(", "));
  }

  findNode(data: { value: number; index: number }) {
    let node = this.head;
    if (node === null) {
      throw new Error("List is empty");
    }
    while (!isDataEqual(node.data, data)) {
      if (node.next === null) {
        throw new Error(`${data} was not found in the list`);
      }
      node = node.next;
    }
    return node;
  }

  move(data: { value: number; index: number }, length: number) {
    const node = this.findNode(data);
    let moveValue = node.data.value % (length - 1);
    let currentNode = node;

    if (moveValue < 0) {
      moveValue += length - 1;
    }

    if (moveValue === 0) {
      return;
    }

    const prevBeforeMove = node.previous;
    const nextBeforeMove = node.next;

    for (let j = 0; j < moveValue; j++) {
      currentNode = currentNode.next;
    }

    prevBeforeMove.next = nextBeforeMove;
    nextBeforeMove.previous = prevBeforeMove;
    node.previous = currentNode;
    node.next = currentNode.next;
    currentNode.next.previous = node;
    currentNode.next = node;
  }
}

const ENCRYPTION_KEY = 811589153;

function main() {
  const input = readFileSync(join(__dirname, "../input.txt"), "utf8");
  const numbers = input
    .split("\n")
    .map((value) => Number(value) * ENCRYPTION_KEY);
  const length = numbers.length;
  const linkedList = new LinkedList();
  numbers.forEach((value, index) => linkedList.insertAtEnd({ index, value }));
  // add last item and close the circular linked list
  const tail = linkedList.getAt(length - 1) as LinkedListNode;
  // @ts-expect-error
  tail.next = linkedList.head;

  // linkedList.printList(length);
  for (let i = 0; i < 10; i++) {
    numbers.forEach((value, index) => {
      linkedList.move({ index, value }, length);
      // linkedList.printList(length);
    });
    // linkedList.printList(length);
  }

  const zeroIndex = linkedList.findIndex({
    index: numbers.findIndex((value) => value === 0),
    value: 0,
  });
  const element1000 = linkedList.getAt((zeroIndex + 1000) % length)?.data
    .value as number;
  const element2000 = linkedList.getAt((zeroIndex + 2000) % length)?.data
    .value as number;
  const element3000 = linkedList.getAt((zeroIndex + 3000) % length)?.data
    .value as number;
  const answer = element1000 + element2000 + element3000;

  console.log(answer);
}

main();

// 7153
