#!../venv/bin/python3
import robot_description_builder as rdb

def main():
	# ==== Material Descriptions ==== #
    blue_material = rdb.MaterialDescriptor()
    print(blue_material)

if __name__=="__main__":
    main()