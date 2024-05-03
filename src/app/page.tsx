'use client';
import Library from "./Library";
import { Provider } from 'react-redux';
import {store} from './store';
import { useSelector } from 'react-redux';


function Libraries(){
	const libraries = useSelector((state: RootState) => state.game.libraries);
	const opponentsDeck = libraries[0];
	const yourDeck = libraries[1];
	return (
		<>
			<Library whos="Opponent" deck={opponentsDeck}/>
			<Library whos="Your" data={yourDeck}/>
		</>
	);
}


export default function Home() {
  return (
		<Provider store={store}>
		<main className="flex min-h-screen flex-col items-center justify-between p-24">
			<Libraries></Libraries>
		</main>
		</Provider>
  );
}
