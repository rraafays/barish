# barish
weather information in the comfort of your terminal from the bbc

## location not working?
bbc weather references locations via an absolutely arbitrary uid,
I have done my best to convert an existing dat table I found from another project however,
any kinds of contributions to src/locations.rs would be much appreciated!

these uids can be obtained by visiting the official bbc weather website and looking up your location,
from there the url will contain its uid as the final word in the string and I would like all locations
to be written in kebab case in src/locations.rs.
