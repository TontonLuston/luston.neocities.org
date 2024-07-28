version_wildcard="#{version}"

version=$1

file="./app/config/sculpin_site_prod.yml"

sed -i "s/$version_wildcard/$version/g" $file
