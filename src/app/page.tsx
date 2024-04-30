'use client';
import Image from "next/image";
import Library from "./Library";

export default function Home() {
	function dothing(){
		console.log(deck);
		console.log(lands)
	}
  return (
		<main className="flex min-h-screen flex-col items-center justify-between p-24">
			<Library/>
		</main>
  );
}
