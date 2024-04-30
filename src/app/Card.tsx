export default function Card({name, color, cost, description, keyName}){
	return(
		<>
		<section className={`${color}-color card library`}>
			<header className="card-header">
				<h1>{name}, {keyName}</h1>
				<h2>{cost}</h2>
			</header>
			<p>{description}</p>
		</section>
		</>
	)
}
