/** @type {import('next').NextConfig} */
const nextConfig = {
	webpack: (config, {isServer, externals})=>{
		// config.externals.push('window');
		// config.output.globalObject = "this";
		return config;
	},
	compiler: {styledComponents: true}


};

export default nextConfig;
