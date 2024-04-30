/** @type {import('next').NextConfig} */
const nextConfig = {
	webpack: (config, {isServer, externals})=>{
		// config.externals.push('window');
		// config.output.globalObject = "this";
		console.log(config)
		return config;
	}

};

export default nextConfig;
